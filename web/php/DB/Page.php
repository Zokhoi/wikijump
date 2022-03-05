<?php

namespace Wikidot\DB;

use Ozone\Framework\Database\Criteria;
use Wikijump\Models\User;

/**
 * Object Model Class.
 *
 */
class Page extends PageBase
{
    public function getMetadata()
    {
        return $this->getCurrentRevision()->getMetadata();
    }

    // TODO: remove
    public function getSource(): string
    {
        // TODO
        return '!!';
    }

    // TODO: remove
    public function getCompiled(): string
    {
        // TODO
        return '!!';
    }

    public function getCurrentRevision()
    {
        $c = new Criteria();
        $c->add("revision_id", $this->getRevisionId());
        return PageRevisionPeer::instance()->selectOne($c);
    }

    public function getFiles()
    {
        $q = "SELECT * FROM file WHERE page_id='" . $this->getPageId() . "' ORDER BY filename, file_id DESC";
        $c = new Criteria();
        $c->setExplicitQuery($q);

        return FilePeer::instance()->select($c);
    }

    public function getCategoryName()
    {
        $unixName = $this->getUnixName();
        if (strpos($unixName, ":") != false) {
            $tmp0 = explode(':', $unixName);
            $categoryName = $tmp0[0];
        } else {
            $categoryName = "_default";
        }
        return $categoryName;
    }

    public function getTitleOrUnixName()
    {
        $title = $this->getTitle();
        if ($title === null || $title === '') {
            $title = ucfirst(str_replace("-", " ", preg_replace("/^[a-z0-9\-]+:/i", '', $this->getUnixName())));
        }
        return $title;
    }

    public function getLastEditUserOrString()
    {
        $user = $this->getLastEditUser();
        if ($user == null) {
            return $this->getLastEditUserString();
        } else {
            return $user;
        }
    }

    public function getLastEditUser()
    {
        if ($this->getLastEditUserId() == User::ANONYMOUS_USER) {
            return null;
        }
        return User::find($this->getLastEditUserId());
    }

    public function getSite()
    {
        return SitePeer::instance()->selectByPrimaryKey($this->getSiteId());
    }
}
